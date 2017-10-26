use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vlddqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 1678766886, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 240, 129, 38, 243, 15, 100], OperandSize::Dword)
}

#[test]
fn vlddqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 371276218, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 240, 36, 205, 186, 57, 33, 22], OperandSize::Qword)
}

#[test]
fn vlddqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 240, 20, 248], OperandSize::Dword)
}

#[test]
fn vlddqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(RDX, 802313016, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 240, 138, 56, 83, 210, 47], OperandSize::Qword)
}

