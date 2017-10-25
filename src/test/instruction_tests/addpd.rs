use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn addpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 219], OperandSize::Dword)
}

fn addpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 627557726, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 154, 94, 197, 103, 37], OperandSize::Dword)
}

fn addpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 227], OperandSize::Qword)
}

fn addpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RSI, 1752855980, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 142, 172, 117, 122, 104], OperandSize::Qword)
}

