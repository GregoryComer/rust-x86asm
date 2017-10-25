use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fimul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 9], OperandSize::Word)
}

fn fimul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1270855105, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 140, 182, 193, 181, 191, 75], OperandSize::Dword)
}

fn fimul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 2119834736, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 140, 254, 112, 28, 90, 126], OperandSize::Qword)
}

fn fimul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(DI, 201, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 141, 201, 0], OperandSize::Word)
}

fn fimul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 12, 254], OperandSize::Dword)
}

fn fimul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(RSI, 493399582, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 142, 30, 174, 104, 29], OperandSize::Qword)
}

