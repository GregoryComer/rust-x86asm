use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 215], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1110984384, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 52, 125, 192, 70, 56, 66], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 220], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 33], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 226], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 149377981, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 140, 152, 189, 83, 231, 8], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 255], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RBX, 1400635178, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 179, 42, 255, 123, 83], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 48, 234], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDX, 1198870854, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 48, 130, 70, 81, 117, 71], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 125, 138, 48, 222], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1241196500, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 140, 48, 172, 152, 212, 39, 251, 73], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 48, 247], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 393024329, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 48, 180, 240, 73, 19, 109, 23], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 172, 48, 194], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM25)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1707165202, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 170, 48, 140, 126, 18, 70, 193, 101], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 48, 220], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 48, 20, 67], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 125, 203, 48, 219], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM14)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 207, 48, 51], OperandSize::Qword)
}

