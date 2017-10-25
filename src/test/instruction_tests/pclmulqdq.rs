use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pclmulqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 226, 122], OperandSize::Dword)
}

fn pclmulqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 920041921, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 60, 189, 193, 185, 214, 54, 63], OperandSize::Dword)
}

fn pclmulqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 237, 101], OperandSize::Qword)
}

fn pclmulqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RSI, 1725641628, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 166, 156, 51, 219, 102, 51], OperandSize::Qword)
}

