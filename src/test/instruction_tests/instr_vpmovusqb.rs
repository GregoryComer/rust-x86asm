use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 18, 205], OperandSize::Dword)
}

#[test]
fn vpmovusqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1634776526, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 18, 60, 189, 206, 181, 112, 97], OperandSize::Dword)
}

#[test]
fn vpmovusqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 126, 143, 18, 203], OperandSize::Qword)
}

#[test]
fn vpmovusqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 18, 52, 208], OperandSize::Qword)
}

#[test]
fn vpmovusqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 18, 192], OperandSize::Dword)
}

#[test]
fn vpmovusqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectDisplaced(ECX, 333845334, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 18, 161, 86, 19, 230, 19], OperandSize::Dword)
}

#[test]
fn vpmovusqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM28)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 126, 174, 18, 220], OperandSize::Qword)
}

#[test]
fn vpmovusqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectDisplaced(RCX, 683744117, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 18, 129, 117, 27, 193, 40], OperandSize::Qword)
}

#[test]
fn vpmovusqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 18, 205], OperandSize::Dword)
}

#[test]
fn vpmovusqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 759865939, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 18, 180, 94, 83, 162, 74, 45], OperandSize::Dword)
}

#[test]
fn vpmovusqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 126, 203, 18, 230], OperandSize::Qword)
}

#[test]
fn vpmovusqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1115914676, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 18, 188, 195, 180, 129, 131, 66], OperandSize::Qword)
}

