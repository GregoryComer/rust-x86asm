use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn divpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 210], OperandSize::Dword)
}

fn divpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 56], OperandSize::Dword)
}

fn divpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 247], OperandSize::Qword)
}

fn divpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 911606433, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 156, 214, 161, 2, 86, 54], OperandSize::Qword)
}

