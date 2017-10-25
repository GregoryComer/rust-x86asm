use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 210, 55], OperandSize::Dword)
}

fn cmpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 2087026289, Some(OperandSize::Dword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 28, 245, 113, 126, 101, 124, 109], OperandSize::Dword)
}

fn cmpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 246, 13], OperandSize::Qword)
}

fn cmpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 2, 0], OperandSize::Qword)
}

