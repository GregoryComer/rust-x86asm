use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ficomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 100, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 89, 100], OperandSize::Word)
}

fn ficomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 28, 144], OperandSize::Dword)
}

fn ficomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectDisplaced(RDX, 460177465, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 154, 57, 192, 109, 27], OperandSize::Qword)
}

fn ficomp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 187, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 155, 187, 0], OperandSize::Word)
}

fn ficomp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 25], OperandSize::Dword)
}

fn ficomp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 26], OperandSize::Qword)
}

