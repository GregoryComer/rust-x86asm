use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lddqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDDQU, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1051308189, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 240, 12, 149, 157, 176, 169, 62], OperandSize::Dword)
}

fn lddqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDDQU, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 493738242, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 240, 148, 194, 2, 217, 109, 29], OperandSize::Qword)
}

