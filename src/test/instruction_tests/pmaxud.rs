use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmaxud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 222], OperandSize::Dword)
}

fn pmaxud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 12, 90], OperandSize::Dword)
}

fn pmaxud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 252], OperandSize::Qword)
}

fn pmaxud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDI, 157828506, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 159, 154, 69, 104, 9], OperandSize::Qword)
}

