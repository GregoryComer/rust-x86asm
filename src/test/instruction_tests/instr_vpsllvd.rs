use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 71, 214], OperandSize::Dword)
}

#[test]
fn vpsllvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 71, 52, 89], OperandSize::Dword)
}

#[test]
fn vpsllvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 71, 255], OperandSize::Qword)
}

#[test]
fn vpsllvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RBX, 1835548393, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 71, 147, 233, 62, 104, 109], OperandSize::Qword)
}

#[test]
fn vpsllvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 71, 225], OperandSize::Dword)
}

#[test]
fn vpsllvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1393383405, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 71, 36, 141, 237, 87, 13, 83], OperandSize::Dword)
}

#[test]
fn vpsllvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 71, 211], OperandSize::Qword)
}

#[test]
fn vpsllvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 71, 44, 186], OperandSize::Qword)
}

#[test]
fn vpsllvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 71, 197], OperandSize::Dword)
}

#[test]
fn vpsllvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 2009882079, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 143, 71, 4, 117, 223, 93, 204, 119], OperandSize::Dword)
}

#[test]
fn vpsllvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 699410810, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 158, 71, 146, 122, 41, 176, 41], OperandSize::Dword)
}

#[test]
fn vpsllvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 61, 143, 71, 203], OperandSize::Qword)
}

#[test]
fn vpsllvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1872408263, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 117, 131, 71, 156, 67, 199, 174, 154, 111], OperandSize::Qword)
}

#[test]
fn vpsllvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 39470050, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 61, 158, 71, 156, 243, 226, 67, 90, 2], OperandSize::Qword)
}

#[test]
fn vpsllvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 173, 71, 245], OperandSize::Dword)
}

#[test]
fn vpsllvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 71, 4, 201], OperandSize::Dword)
}

#[test]
fn vpsllvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 418123413, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 71, 60, 117, 149, 14, 236, 24], OperandSize::Dword)
}

#[test]
fn vpsllvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 93, 175, 71, 208], OperandSize::Qword)
}

#[test]
fn vpsllvd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 29, 175, 71, 4, 177], OperandSize::Qword)
}

#[test]
fn vpsllvd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RAX, 453664094, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 125, 183, 71, 128, 94, 93, 10, 27], OperandSize::Qword)
}

#[test]
fn vpsllvd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 201, 71, 250], OperandSize::Dword)
}

#[test]
fn vpsllvd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1449063951, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 206, 71, 28, 77, 15, 246, 94, 86], OperandSize::Dword)
}

#[test]
fn vpsllvd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 223, 71, 4, 217], OperandSize::Dword)
}

#[test]
fn vpsllvd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 77, 207, 71, 212], OperandSize::Qword)
}

#[test]
fn vpsllvd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 109, 199, 71, 17], OperandSize::Qword)
}

#[test]
fn vpsllvd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1525160995, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 53, 217, 71, 188, 112, 35, 28, 232, 90], OperandSize::Qword)
}

