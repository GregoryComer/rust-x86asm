use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 71, 245], OperandSize::Dword)
}

#[test]
fn vpsllvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 71, 35], OperandSize::Dword)
}

#[test]
fn vpsllvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 71, 200], OperandSize::Qword)
}

#[test]
fn vpsllvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 71, 4, 223], OperandSize::Qword)
}

#[test]
fn vpsllvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 71, 227], OperandSize::Dword)
}

#[test]
fn vpsllvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 71, 33], OperandSize::Dword)
}

#[test]
fn vpsllvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 71, 193], OperandSize::Qword)
}

#[test]
fn vpsllvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDI, 1204651613, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 71, 191, 93, 134, 205, 71], OperandSize::Qword)
}

#[test]
fn vpsllvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 71, 211], OperandSize::Dword)
}

#[test]
fn vpsllvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 137, 71, 40], OperandSize::Dword)
}

#[test]
fn vpsllvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 327926290, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 153, 71, 52, 125, 18, 194, 139, 19], OperandSize::Dword)
}

#[test]
fn vpsllvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 71, 243], OperandSize::Qword)
}

#[test]
fn vpsllvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM17)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 117, 135, 71, 56], OperandSize::Qword)
}

#[test]
fn vpsllvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RDI, 1817252321, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 21, 156, 71, 159, 225, 17, 81, 108], OperandSize::Qword)
}

#[test]
fn vpsllvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 169, 71, 205], OperandSize::Dword)
}

#[test]
fn vpsllvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 2066086670, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 71, 132, 241, 14, 251, 37, 123], OperandSize::Dword)
}

#[test]
fn vpsllvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 995774757, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 191, 71, 183, 37, 81, 90, 59], OperandSize::Dword)
}

#[test]
fn vpsllvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 85, 161, 71, 202], OperandSize::Qword)
}

#[test]
fn vpsllvd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1898375017, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 5, 175, 71, 164, 73, 105, 231, 38, 113], OperandSize::Qword)
}

#[test]
fn vpsllvd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1356555966, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 29, 180, 71, 44, 213, 190, 102, 219, 80], OperandSize::Qword)
}

#[test]
fn vpsllvd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 206, 71, 218], OperandSize::Dword)
}

#[test]
fn vpsllvd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ECX, 65411935, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 205, 71, 153, 95, 27, 230, 3], OperandSize::Dword)
}

#[test]
fn vpsllvd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 2043493560, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 218, 71, 164, 127, 184, 60, 205, 121], OperandSize::Dword)
}

#[test]
fn vpsllvd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 21, 204, 71, 197], OperandSize::Qword)
}

#[test]
fn vpsllvd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectDisplaced(RSI, 773549109, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 195, 71, 142, 53, 108, 27, 46], OperandSize::Qword)
}

#[test]
fn vpsllvd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 37, 212, 71, 57], OperandSize::Qword)
}

