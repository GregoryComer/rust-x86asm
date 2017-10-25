use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 235], OperandSize::Dword)
}

fn cvtpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 14], OperandSize::Dword)
}

fn cvtpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 207], OperandSize::Qword)
}

fn cvtpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RCX, 241444696, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 169, 88, 39, 100, 14], OperandSize::Qword)
}

