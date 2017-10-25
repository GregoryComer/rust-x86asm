use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtpd2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 217], OperandSize::Dword)
}

fn cvtpd2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 60, 193], OperandSize::Dword)
}

fn cvtpd2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 255], OperandSize::Qword)
}

fn cvtpd2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1572427148, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 28, 205, 140, 85, 185, 93], OperandSize::Qword)
}

