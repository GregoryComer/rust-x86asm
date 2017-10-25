use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn haddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 215], OperandSize::Dword)
}

fn haddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EBX, 1126657778, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 155, 242, 110, 39, 67], OperandSize::Dword)
}

fn haddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 218], OperandSize::Qword)
}

fn haddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 9240050, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 4, 213, 242, 253, 140, 0], OperandSize::Qword)
}

