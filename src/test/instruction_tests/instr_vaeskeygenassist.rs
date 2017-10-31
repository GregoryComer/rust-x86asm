use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaeskeygenassist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 238, 13], OperandSize::Dword)
}

#[test]
fn vaeskeygenassist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1478561338, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 172, 246, 58, 14, 33, 88, 53], OperandSize::Dword)
}

#[test]
fn vaeskeygenassist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 211, 100], OperandSize::Qword)
}

#[test]
fn vaeskeygenassist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RCX, 1759431541, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 169, 117, 203, 222, 104, 51], OperandSize::Qword)
}

