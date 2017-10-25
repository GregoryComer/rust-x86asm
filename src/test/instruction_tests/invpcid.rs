use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn invpcid_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INVPCID, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1354051600, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 130, 172, 81, 16, 48, 181, 80], OperandSize::Dword)
}

fn invpcid_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INVPCID, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 2097517427, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 130, 188, 144, 115, 147, 5, 125], OperandSize::Qword)
}

