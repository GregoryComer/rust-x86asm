use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaesdec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 222, 231], OperandSize::Dword)
}

fn vaesdec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 1901778899, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 222, 131, 211, 215, 90, 113], OperandSize::Dword)
}

fn vaesdec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 222, 215], OperandSize::Qword)
}

fn vaesdec_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1769618056, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 222, 172, 150, 136, 58, 122, 105], OperandSize::Qword)
}

