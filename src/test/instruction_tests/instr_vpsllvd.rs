use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 71, 197], OperandSize::Dword)
}

#[test]
fn vpsllvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 71, 41], OperandSize::Dword)
}

#[test]
fn vpsllvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 71, 221], OperandSize::Qword)
}

#[test]
fn vpsllvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 71, 28, 254], OperandSize::Qword)
}

#[test]
fn vpsllvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 71, 209], OperandSize::Dword)
}

#[test]
fn vpsllvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1020227261, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 71, 44, 125, 189, 110, 207, 60], OperandSize::Dword)
}

#[test]
fn vpsllvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 71, 212], OperandSize::Qword)
}

#[test]
fn vpsllvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 71, 11], OperandSize::Qword)
}

#[test]
fn vpsllvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 140, 71, 204], OperandSize::Dword)
}

#[test]
fn vpsllvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 71, 43], OperandSize::Dword)
}

#[test]
fn vpsllvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 154, 71, 36, 123], OperandSize::Dword)
}

#[test]
fn vpsllvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 53, 130, 71, 194], OperandSize::Qword)
}

#[test]
fn vpsllvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 71, 44, 67], OperandSize::Qword)
}

#[test]
fn vpsllvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 101, 153, 71, 4, 243], OperandSize::Qword)
}

#[test]
fn vpsllvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 174, 71, 201], OperandSize::Dword)
}

#[test]
fn vpsllvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 333838751, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 71, 162, 159, 249, 229, 19], OperandSize::Dword)
}

#[test]
fn vpsllvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 101204009, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 189, 71, 170, 41, 64, 8, 6], OperandSize::Dword)
}

#[test]
fn vpsllvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 125, 166, 71, 209], OperandSize::Qword)
}

#[test]
fn vpsllvd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 432602174, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 77, 170, 71, 44, 245, 62, 252, 200, 25], OperandSize::Qword)
}

#[test]
fn vpsllvd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 73335356, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 117, 179, 71, 52, 189, 60, 2, 95, 4], OperandSize::Qword)
}

#[test]
fn vpsllvd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 202, 71, 227], OperandSize::Dword)
}

#[test]
fn vpsllvd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 360065070, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 203, 71, 60, 245, 46, 40, 118, 21], OperandSize::Dword)
}

#[test]
fn vpsllvd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 218, 71, 34], OperandSize::Dword)
}

#[test]
fn vpsllvd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 101, 193, 71, 222], OperandSize::Qword)
}

#[test]
fn vpsllvd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 21, 201, 71, 60, 191], OperandSize::Qword)
}

#[test]
fn vpsllvd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(RBX, 1270891631, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 220, 71, 139, 111, 68, 192, 75], OperandSize::Qword)
}

