use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 50, 243], OperandSize::Dword)
}

#[test]
fn vpmovqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledDisplaced(ESI, Four, 84303442, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 50, 12, 181, 82, 94, 6, 5], OperandSize::Dword)
}

#[test]
fn vpmovqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 126, 143, 50, 196], OperandSize::Qword)
}

#[test]
fn vpmovqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 50, 44, 134], OperandSize::Qword)
}

#[test]
fn vpmovqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 50, 224], OperandSize::Dword)
}

#[test]
fn vpmovqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1314545106, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 50, 164, 73, 210, 93, 90, 78], OperandSize::Dword)
}

#[test]
fn vpmovqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 50, 236], OperandSize::Qword)
}

#[test]
fn vpmovqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1186213976, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 50, 20, 205, 88, 48, 180, 70], OperandSize::Qword)
}

#[test]
fn vpmovqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 50, 200], OperandSize::Dword)
}

#[test]
fn vpmovqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledDisplaced(ECX, Four, 354167859, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 50, 28, 141, 51, 44, 28, 21], OperandSize::Dword)
}

#[test]
fn vpmovqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 126, 206, 50, 211], OperandSize::Qword)
}

#[test]
fn vpmovqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledDisplaced(RDX, Two, 542243312, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 50, 4, 85, 240, 249, 81, 32], OperandSize::Qword)
}

