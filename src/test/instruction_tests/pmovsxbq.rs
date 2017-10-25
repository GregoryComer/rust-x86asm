use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovsxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 199], OperandSize::Dword)
}

fn pmovsxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 59], OperandSize::Dword)
}

fn pmovsxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 192], OperandSize::Qword)
}

fn pmovsxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 518983620, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 34, 164, 199, 196, 15, 239, 30], OperandSize::Qword)
}

