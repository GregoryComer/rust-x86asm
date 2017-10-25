use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 52, 253], OperandSize::Dword)
}

#[test]
fn vpmovqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectDisplaced(EDX, 1696510708, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 52, 138, 244, 178, 30, 101], OperandSize::Dword)
}

#[test]
fn vpmovqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 126, 137, 52, 207], OperandSize::Qword)
}

#[test]
fn vpmovqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 52, 12, 152], OperandSize::Qword)
}

#[test]
fn vpmovqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 52, 239], OperandSize::Dword)
}

#[test]
fn vpmovqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1227771199, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 52, 60, 141, 63, 77, 46, 73], OperandSize::Dword)
}

#[test]
fn vpmovqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 126, 174, 52, 250], OperandSize::Qword)
}

#[test]
fn vpmovqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 581895236, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 52, 12, 213, 68, 4, 175, 34], OperandSize::Qword)
}

#[test]
fn vpmovqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 52, 242], OperandSize::Dword)
}

#[test]
fn vpmovqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 52, 40], OperandSize::Dword)
}

#[test]
fn vpmovqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 126, 207, 52, 223], OperandSize::Qword)
}

#[test]
fn vpmovqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledDisplaced(RDX, Two, 932338222, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 52, 60, 85, 46, 90, 146, 55], OperandSize::Qword)
}

