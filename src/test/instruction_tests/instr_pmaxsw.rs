use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 192], OperandSize::Dword)
}

#[test]
fn pmaxsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(EDX, 937535651, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 146, 163, 168, 225, 55], OperandSize::Dword)
}

#[test]
fn pmaxsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 208], OperandSize::Qword)
}

#[test]
fn pmaxsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 36, 203], OperandSize::Qword)
}

#[test]
fn pmaxsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 233], OperandSize::Dword)
}

#[test]
fn pmaxsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 1861940091, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 158, 123, 243, 250, 110], OperandSize::Dword)
}

#[test]
fn pmaxsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 195], OperandSize::Qword)
}

#[test]
fn pmaxsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 7], OperandSize::Qword)
}

