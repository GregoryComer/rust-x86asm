use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 18, 210], OperandSize::Dword)
}

#[test]
fn vpmovusqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 18, 44, 159], OperandSize::Dword)
}

#[test]
fn vpmovusqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 126, 142, 18, 208], OperandSize::Qword)
}

#[test]
fn vpmovusqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 18, 44, 73], OperandSize::Qword)
}

#[test]
fn vpmovusqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 18, 251], OperandSize::Dword)
}

#[test]
fn vpmovusqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 18, 28, 75], OperandSize::Dword)
}

#[test]
fn vpmovusqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM20)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 126, 169, 18, 204], OperandSize::Qword)
}

#[test]
fn vpmovusqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1850055612, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 18, 180, 223, 188, 155, 69, 110], OperandSize::Qword)
}

#[test]
fn vpmovusqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 18, 229], OperandSize::Dword)
}

#[test]
fn vpmovusqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 4085111, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 18, 132, 75, 119, 85, 62, 0], OperandSize::Dword)
}

#[test]
fn vpmovusqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM27)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 126, 203, 18, 243], OperandSize::Qword)
}

#[test]
fn vpmovusqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 76454727, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 18, 28, 197, 71, 155, 142, 4], OperandSize::Qword)
}

