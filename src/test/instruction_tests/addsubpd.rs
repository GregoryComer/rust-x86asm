use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn addsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 209], OperandSize::Dword)
}

fn addsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1511577809, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 132, 251, 209, 216, 24, 90], OperandSize::Dword)
}

fn addsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 205], OperandSize::Qword)
}

fn addsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 7], OperandSize::Qword)
}

