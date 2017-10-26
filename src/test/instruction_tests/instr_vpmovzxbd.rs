use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 243], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 168754699, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 36, 189, 11, 254, 14, 10], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 195], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 36, 142], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 243], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 46], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 195], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 60, 74], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 49, 220], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EBX, 1761664001, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 49, 171, 1, 220, 0, 105], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 125, 137, 49, 226], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 139, 49, 28, 129], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 49, 244], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1454832628, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 49, 164, 254, 244, 251, 182, 86], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 170, 49, 226], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 49, 12, 81], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 49, 204], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1426240585, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 49, 180, 150, 73, 180, 2, 85], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 49, 236], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 49, 40], OperandSize::Qword)
}

