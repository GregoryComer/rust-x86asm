use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn unpcklpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 205], OperandSize::Dword)
}

fn unpcklpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 363484778, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 172, 66, 106, 86, 170, 21], OperandSize::Dword)
}

fn unpcklpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 207], OperandSize::Qword)
}

fn unpcklpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 2056985, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 52, 125, 25, 99, 31, 0], OperandSize::Qword)
}

