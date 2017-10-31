use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovwb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 143, 48, 232], OperandSize::Dword)
}

#[test]
fn vpmovwb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectDisplaced(ECX, 88491811, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 48, 161, 35, 71, 70, 5], OperandSize::Dword)
}

#[test]
fn vpmovwb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 126, 140, 48, 207], OperandSize::Qword)
}

#[test]
fn vpmovwb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectDisplaced(RCX, 1087354993, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 48, 153, 113, 184, 207, 64], OperandSize::Qword)
}

#[test]
fn vpmovwb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 48, 253], OperandSize::Dword)
}

#[test]
fn vpmovwb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 188219959, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 48, 148, 146, 55, 2, 56, 11], OperandSize::Dword)
}

#[test]
fn vpmovwb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM30)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 126, 172, 48, 254], OperandSize::Qword)
}

#[test]
fn vpmovwb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledDisplaced(RCX, Four, 549126518, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 48, 20, 141, 118, 1, 187, 32], OperandSize::Qword)
}

#[test]
fn vpmovwb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 48, 197], OperandSize::Dword)
}

#[test]
fn vpmovwb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 48, 36, 251], OperandSize::Dword)
}

#[test]
fn vpmovwb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(YMM28)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 126, 202, 48, 212], OperandSize::Qword)
}

#[test]
fn vpmovwb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledDisplaced(RAX, Four, 2051183937, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 48, 44, 133, 65, 149, 66, 122], OperandSize::Qword)
}

