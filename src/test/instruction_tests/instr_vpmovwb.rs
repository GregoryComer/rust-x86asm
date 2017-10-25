use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovwb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 48, 194], OperandSize::Dword)
}

#[test]
fn vpmovwb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 48, 24], OperandSize::Dword)
}

#[test]
fn vpmovwb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 126, 142, 48, 250], OperandSize::Qword)
}

#[test]
fn vpmovwb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectDisplaced(RSI, 292823761, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 48, 150, 209, 34, 116, 17], OperandSize::Qword)
}

#[test]
fn vpmovwb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 48, 201], OperandSize::Dword)
}

#[test]
fn vpmovwb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectDisplaced(ECX, 1829849168, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 48, 185, 80, 72, 17, 109], OperandSize::Dword)
}

#[test]
fn vpmovwb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM26)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 126, 171, 48, 226], OperandSize::Qword)
}

#[test]
fn vpmovwb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 48, 60, 122], OperandSize::Qword)
}

#[test]
fn vpmovwb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 204, 48, 204], OperandSize::Dword)
}

#[test]
fn vpmovwb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1433769372, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 48, 60, 133, 156, 149, 117, 85], OperandSize::Dword)
}

#[test]
fn vpmovwb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(YMM18)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 126, 203, 48, 242], OperandSize::Qword)
}

#[test]
fn vpmovwb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 48, 44, 159], OperandSize::Qword)
}

