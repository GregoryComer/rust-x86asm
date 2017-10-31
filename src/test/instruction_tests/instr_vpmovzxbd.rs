use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 202], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 9], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 236], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RAX, 863561283, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 136, 67, 230, 120, 51], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 214], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EBX, 567011635, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 147, 51, 233, 203, 33], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 230], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1838663867, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 148, 113, 187, 200, 151, 109], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 49, 215], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ESI, 2087603368, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 49, 134, 168, 76, 110, 124], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 125, 141, 49, 208], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 718342597, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 137, 49, 188, 137, 197, 9, 209, 42], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 49, 214], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 49, 30], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 125, 169, 49, 215], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 108116629, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 172, 49, 172, 72, 149, 186, 113, 6], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 49, 223], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 49, 60, 192], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 125, 204, 49, 219], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectDisplaced(RBX, 677814842, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 206, 49, 131, 58, 162, 102, 40], OperandSize::Qword)
}

