use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn mulss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 197], OperandSize::Dword)
}

fn mulss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 2104671718, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 172, 90, 230, 189, 114, 125], OperandSize::Dword)
}

fn mulss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 226], OperandSize::Qword)
}

fn mulss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 60, 80], OperandSize::Qword)
}

