use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn shufps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 209, 11], OperandSize::Dword)
}

fn shufps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 28, 123, 52], OperandSize::Dword)
}

fn shufps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 220, 75], OperandSize::Qword)
}

fn shufps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 20, 218, 41], OperandSize::Qword)
}

