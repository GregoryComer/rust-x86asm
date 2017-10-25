use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 194, 206, 16], OperandSize::Dword)
}

#[test]
fn vcmpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 194, 60, 176, 26], OperandSize::Dword)
}

#[test]
fn vcmpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 194, 210, 76], OperandSize::Qword)
}

#[test]
fn vcmpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1322100099, Some(OperandSize::Qword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 194, 132, 248, 131, 165, 205, 78, 28], OperandSize::Qword)
}

#[test]
fn vcmpsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 207, 31, 194, 230, 98], OperandSize::Dword)
}

#[test]
fn vcmpsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 275734535, Some(OperandSize::Qword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 223, 14, 194, 60, 77, 7, 96, 111, 16, 114], OperandSize::Dword)
}

#[test]
fn vcmpsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM27)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 207, 28, 194, 251, 33], OperandSize::Qword)
}

#[test]
fn vcmpsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RBX, 2112416897, Some(OperandSize::Qword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 183, 7, 194, 187, 129, 236, 232, 125, 99], OperandSize::Qword)
}

