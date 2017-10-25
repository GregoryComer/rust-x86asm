use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 139, 32, 222], OperandSize::Dword)
}

#[test]
fn vpmovswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1080908154, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 32, 164, 241, 122, 89, 109, 64], OperandSize::Dword)
}

#[test]
fn vpmovswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 126, 140, 32, 253], OperandSize::Qword)
}

#[test]
fn vpmovswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 32, 60, 65], OperandSize::Qword)
}

#[test]
fn vpmovswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 32, 253], OperandSize::Dword)
}

#[test]
fn vpmovswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 32, 51], OperandSize::Dword)
}

#[test]
fn vpmovswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM30)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 126, 175, 32, 238], OperandSize::Qword)
}

#[test]
fn vpmovswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 32, 51], OperandSize::Qword)
}

#[test]
fn vpmovswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 32, 244], OperandSize::Dword)
}

#[test]
fn vpmovswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledDisplaced(EDX, Two, 383700044, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 32, 36, 85, 76, 204, 222, 22], OperandSize::Dword)
}

#[test]
fn vpmovswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(YMM26)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 126, 205, 32, 226], OperandSize::Qword)
}

#[test]
fn vpmovswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 32, 60, 183], OperandSize::Qword)
}

