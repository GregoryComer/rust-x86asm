use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 253], OperandSize::Dword)
}

#[test]
fn pmaxsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(EAX, 1717020765, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 128, 93, 168, 87, 102], OperandSize::Dword)
}

#[test]
fn pmaxsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 252], OperandSize::Qword)
}

#[test]
fn pmaxsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 36, 208], OperandSize::Qword)
}

#[test]
fn pmaxsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 254], OperandSize::Dword)
}

#[test]
fn pmaxsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 36, 113], OperandSize::Dword)
}

#[test]
fn pmaxsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 196], OperandSize::Qword)
}

#[test]
fn pmaxsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1827683966, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 156, 66, 126, 62, 240, 108], OperandSize::Qword)
}

