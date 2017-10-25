use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmaskmovps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 122115054, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 44, 44, 69, 238, 83, 71, 7], OperandSize::Dword)
}

fn vmaskmovps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 44, 3], OperandSize::Qword)
}

fn vmaskmovps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 44, 12, 120], OperandSize::Dword)
}

fn vmaskmovps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 44, 3], OperandSize::Qword)
}

fn vmaskmovps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 46, 42], OperandSize::Dword)
}

fn vmaskmovps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 455656941, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 46, 148, 211, 237, 197, 40, 27], OperandSize::Qword)
}

fn vmaskmovps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectDisplaced(ECX, 172861139, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 46, 185, 211, 166, 77, 10], OperandSize::Dword)
}

fn vmaskmovps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1465922662, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 46, 132, 90, 102, 52, 96, 87], OperandSize::Qword)
}

