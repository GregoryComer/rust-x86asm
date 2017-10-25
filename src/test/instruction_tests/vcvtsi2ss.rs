use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtsi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 42, 255], OperandSize::Dword)
}

fn vcvtsi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 42, 42], OperandSize::Dword)
}

fn vcvtsi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 42, 234], OperandSize::Qword)
}

fn vcvtsi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 101628942, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 42, 164, 195, 14, 188, 14, 6], OperandSize::Qword)
}

fn vcvtsi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 226, 42, 198], OperandSize::Qword)
}

fn vcvtsi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDI, 424754945, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 234, 42, 143, 1, 63, 81, 25], OperandSize::Qword)
}

fn vcvtsi2ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 70, 24, 42, 236], OperandSize::Dword)
}

fn vcvtsi2ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 42, 48], OperandSize::Dword)
}

fn vcvtsi2ss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 102, 112, 42, 233], OperandSize::Qword)
}

fn vcvtsi2ss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 388646754, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 6, 8, 42, 52, 85, 98, 71, 42, 23], OperandSize::Qword)
}

fn vcvtsi2ss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 198, 56, 42, 237], OperandSize::Qword)
}

fn vcvtsi2ss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 190, 0, 42, 14], OperandSize::Qword)
}

