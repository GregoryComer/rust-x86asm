use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpistrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 253, 39], OperandSize::Dword)
}

fn vpcmpistrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EBX, 1248272946, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 171, 50, 34, 103, 74, 115], OperandSize::Dword)
}

fn vpcmpistrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 196, 14], OperandSize::Qword)
}

fn vpcmpistrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 85643390, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 156, 83, 126, 208, 26, 5, 13], OperandSize::Qword)
}

