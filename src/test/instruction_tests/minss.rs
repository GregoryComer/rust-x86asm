use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn minss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 217], OperandSize::Dword)
}

fn minss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 38], OperandSize::Dword)
}

fn minss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 242], OperandSize::Qword)
}

fn minss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1473006196, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 36, 149, 116, 74, 204, 87], OperandSize::Qword)
}

