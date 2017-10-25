use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lds_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(DX)), operand2: Some(Indirect(DI, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 21], OperandSize::Word)
}

#[test]
fn lds_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(ECX, 1918415749, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 197, 169, 133, 179, 88, 114], OperandSize::Dword)
}

#[test]
fn lds_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 1050793643, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 140, 137, 171, 214, 161, 62], OperandSize::Dword)
}

