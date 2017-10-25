use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fimul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(BX, 4652, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 143, 44, 18], OperandSize::Word)
}

#[test]
fn fimul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1191277820, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 12, 77, 252, 116, 1, 71], OperandSize::Dword)
}

#[test]
fn fimul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(RCX, 622117513, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 137, 137, 194, 20, 37], OperandSize::Qword)
}

#[test]
fn fimul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(DI, 65, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 77, 65], OperandSize::Word)
}

#[test]
fn fimul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 12, 246], OperandSize::Dword)
}

#[test]
fn fimul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledDisplaced(RDI, Four, 400593183, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 12, 189, 31, 145, 224, 23], OperandSize::Qword)
}

