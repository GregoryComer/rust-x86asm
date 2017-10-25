use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movntpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPD, operand1: Some(IndirectScaledDisplaced(EBX, Four, 1902552429, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 43, 4, 157, 109, 165, 102, 113], OperandSize::Dword)
}

fn movntpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPD, operand1: Some(IndirectScaledDisplaced(RBX, Four, 2023682490, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 43, 28, 157, 186, 241, 158, 120], OperandSize::Qword)
}

