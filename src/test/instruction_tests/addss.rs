use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn addss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 235], OperandSize::Dword)
}

fn addss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 801776712, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 188, 87, 72, 36, 202, 47], OperandSize::Dword)
}

fn addss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 204], OperandSize::Qword)
}

fn addss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 35], OperandSize::Qword)
}

