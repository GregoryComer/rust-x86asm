use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn minsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 192], OperandSize::Dword)
}

fn minsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1032087214, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 140, 86, 174, 102, 132, 61], OperandSize::Dword)
}

fn minsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 250], OperandSize::Qword)
}

fn minsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1637158028, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 188, 82, 140, 12, 149, 97], OperandSize::Qword)
}

