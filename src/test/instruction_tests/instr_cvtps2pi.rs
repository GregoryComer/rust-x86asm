use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 194], OperandSize::Word)
}

#[test]
fn cvtps2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(Indirect(BX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 39], OperandSize::Word)
}

#[test]
fn cvtps2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 193], OperandSize::Dword)
}

#[test]
fn cvtps2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 4, 254], OperandSize::Dword)
}

#[test]
fn cvtps2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 208], OperandSize::Qword)
}

#[test]
fn cvtps2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM1)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 9], OperandSize::Qword)
}

