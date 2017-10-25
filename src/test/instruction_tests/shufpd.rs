use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn shufpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 222, 3], OperandSize::Dword)
}

fn shufpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 44, 190, 43], OperandSize::Dword)
}

fn shufpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 216, 118], OperandSize::Qword)
}

fn shufpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 8, 43], OperandSize::Qword)
}

