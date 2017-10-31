use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lgs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(DI, 90, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 85, 90], OperandSize::Word)
}

#[test]
fn lgs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 181, 28, 120], OperandSize::Dword)
}

#[test]
fn lgs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 618905129, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 181, 12, 213, 41, 190, 227, 36], OperandSize::Qword)
}

#[test]
fn lgs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 52, 129], OperandSize::Dword)
}

#[test]
fn lgs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1416678466, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 44, 213, 66, 204, 112, 84], OperandSize::Qword)
}

#[test]
fn lgs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1863571290, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 181, 20, 189, 90, 215, 19, 111], OperandSize::Qword)
}

