use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtpd2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 253], OperandSize::Word)
}

fn cvtpd2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(DI, 248, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 173, 248, 0], OperandSize::Word)
}

fn cvtpd2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 242], OperandSize::Dword)
}

fn cvtpd2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 12, 71], OperandSize::Dword)
}

fn cvtpd2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 240], OperandSize::Qword)
}

fn cvtpd2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 12, 143], OperandSize::Qword)
}

