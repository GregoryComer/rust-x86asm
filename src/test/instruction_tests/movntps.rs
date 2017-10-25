use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movntps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPS, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 43, 38], OperandSize::Dword)
}

fn movntps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPS, operand1: Some(IndirectDisplaced(RDI, 482266577, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 43, 135, 209, 205, 190, 28], OperandSize::Qword)
}

