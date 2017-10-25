use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movlpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 18, 54], OperandSize::Dword)
}

fn movlpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 18, 36, 207], OperandSize::Qword)
}

fn movlpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 19, 60, 126], OperandSize::Dword)
}

fn movlpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 19, 34], OperandSize::Qword)
}

