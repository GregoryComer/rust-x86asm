use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vextractps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 241, 77], OperandSize::Dword)
}

fn vextractps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 16, 87], OperandSize::Dword)
}

fn vextractps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 242, 37], OperandSize::Qword)
}

fn vextractps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 33, 103], OperandSize::Qword)
}

fn vextractps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 210, 7], OperandSize::Dword)
}

fn vextractps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1909306911, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 164, 86, 31, 182, 205, 113, 108], OperandSize::Dword)
}

fn vextractps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 196, 15], OperandSize::Qword)
}

fn vextractps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1501099969, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM8)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 121, 23, 4, 157, 193, 247, 120, 89, 66], OperandSize::Qword)
}

