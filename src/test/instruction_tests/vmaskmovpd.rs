use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmaskmovpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 564666879, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 45, 140, 126, 255, 33, 168, 33], OperandSize::Dword)
}

fn vmaskmovpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 45, 25], OperandSize::Qword)
}

fn vmaskmovpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 941919641, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 45, 60, 69, 153, 141, 36, 56], OperandSize::Dword)
}

fn vmaskmovpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 1058101058, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 45, 146, 66, 87, 17, 63], OperandSize::Qword)
}

fn vmaskmovpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 47, 23], OperandSize::Dword)
}

fn vmaskmovpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectDisplaced(RCX, 466151566, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 47, 169, 142, 232, 200, 27], OperandSize::Qword)
}

fn vmaskmovpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectDisplaced(ECX, 886056311, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 47, 161, 119, 37, 208, 52], OperandSize::Dword)
}

fn vmaskmovpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 47, 49], OperandSize::Qword)
}

