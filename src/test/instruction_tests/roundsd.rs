use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn roundsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 192, 73], OperandSize::Dword)
}

fn roundsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 455015242, Some(OperandSize::Qword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 176, 74, 251, 30, 27, 66], OperandSize::Dword)
}

fn roundsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 249, 21], OperandSize::Qword)
}

fn roundsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1515642711, Some(OperandSize::Qword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 180, 202, 87, 223, 86, 90, 102], OperandSize::Qword)
}

