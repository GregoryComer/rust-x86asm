use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn maxpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 245], OperandSize::Dword)
}

fn maxpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 550510135, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 148, 114, 55, 30, 208, 32], OperandSize::Dword)
}

fn maxpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 230], OperandSize::Qword)
}

fn maxpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 4, 249], OperandSize::Qword)
}

