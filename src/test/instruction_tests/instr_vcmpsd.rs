use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 194, 205, 88], OperandSize::Dword)
}

#[test]
fn vcmpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1507781355, Some(OperandSize::Qword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 194, 52, 205, 235, 234, 222, 89, 31], OperandSize::Dword)
}

#[test]
fn vcmpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 194, 196, 70], OperandSize::Qword)
}

#[test]
fn vcmpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 194, 36, 83, 4], OperandSize::Qword)
}

#[test]
fn vcmpsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 223, 28, 194, 214, 68], OperandSize::Dword)
}

#[test]
fn vcmpsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 215, 12, 194, 12, 206, 27], OperandSize::Dword)
}

#[test]
fn vcmpsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM9)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 199, 26, 194, 217, 101], OperandSize::Qword)
}

#[test]
fn vcmpsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RSI, 341844329, Some(OperandSize::Qword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 191, 2, 194, 166, 105, 33, 96, 20, 53], OperandSize::Qword)
}

