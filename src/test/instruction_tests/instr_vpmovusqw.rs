use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 20, 231], OperandSize::Dword)
}

#[test]
fn vpmovusqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 20, 16], OperandSize::Dword)
}

#[test]
fn vpmovusqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 126, 138, 20, 212], OperandSize::Qword)
}

#[test]
fn vpmovusqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectDisplaced(RBX, 1212203636, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 20, 131, 116, 194, 64, 72], OperandSize::Qword)
}

#[test]
fn vpmovusqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 20, 237], OperandSize::Dword)
}

#[test]
fn vpmovusqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1940223468, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 20, 44, 221, 236, 117, 165, 115], OperandSize::Dword)
}

#[test]
fn vpmovusqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 126, 173, 20, 225], OperandSize::Qword)
}

#[test]
fn vpmovusqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 20, 4, 87], OperandSize::Qword)
}

#[test]
fn vpmovusqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 204, 20, 228], OperandSize::Dword)
}

#[test]
fn vpmovusqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1600204155, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 20, 156, 87, 123, 45, 97, 95], OperandSize::Dword)
}

#[test]
fn vpmovusqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 126, 204, 20, 213], OperandSize::Qword)
}

#[test]
fn vpmovusqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectDisplaced(RDI, 1784209058, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 20, 175, 162, 222, 88, 106], OperandSize::Qword)
}

