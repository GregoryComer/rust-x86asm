use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 139, 50, 233], OperandSize::Dword)
}

#[test]
fn vpmovqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 50, 2], OperandSize::Dword)
}

#[test]
fn vpmovqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 126, 142, 50, 214], OperandSize::Qword)
}

#[test]
fn vpmovqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 77072782, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 50, 156, 209, 142, 9, 152, 4], OperandSize::Qword)
}

#[test]
fn vpmovqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 50, 214], OperandSize::Dword)
}

#[test]
fn vpmovqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1516501134, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 50, 180, 119, 142, 248, 99, 90], OperandSize::Dword)
}

#[test]
fn vpmovqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM23)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 126, 172, 50, 231], OperandSize::Qword)
}

#[test]
fn vpmovqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 50, 3], OperandSize::Qword)
}

#[test]
fn vpmovqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 50, 206], OperandSize::Dword)
}

#[test]
fn vpmovqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledDisplaced(ECX, Two, 370226681, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 50, 4, 77, 249, 53, 17, 22], OperandSize::Dword)
}

#[test]
fn vpmovqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 126, 207, 50, 207], OperandSize::Qword)
}

#[test]
fn vpmovqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 50, 44, 135], OperandSize::Qword)
}

