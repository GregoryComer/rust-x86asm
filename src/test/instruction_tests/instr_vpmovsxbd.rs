use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 223], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1337822250, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 52, 189, 42, 140, 189, 79], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 220], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 2004622591, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 60, 133, 255, 28, 124, 119], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 233], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1397231358, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 132, 243, 254, 14, 72, 83], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 217], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 3], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 33, 236], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 33, 44, 129], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 125, 138, 33, 232], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 1605757900, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 141, 33, 132, 243, 204, 235, 181, 95], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 33, 229], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 221059585, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 33, 60, 189, 1, 26, 45, 13], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 33, 206], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 33, 20, 74], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 33, 193], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1055419374, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 33, 60, 181, 238, 107, 232, 62], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 125, 202, 33, 230], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1407900966, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 33, 12, 245, 38, 221, 234, 83], OperandSize::Qword)
}

