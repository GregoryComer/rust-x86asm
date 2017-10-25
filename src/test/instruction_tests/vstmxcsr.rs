use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vstmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSTMXCSR, operand1: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 28, 154], OperandSize::Dword)
}

fn vstmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSTMXCSR, operand1: Some(IndirectDisplaced(RDX, 699095710, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 154, 158, 90, 171, 41], OperandSize::Qword)
}

