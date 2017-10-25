use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaeskeygenassist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 225, 112], OperandSize::Dword)
}

fn vaeskeygenassist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDI, 461220152, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 151, 56, 169, 125, 27, 33], OperandSize::Dword)
}

fn vaeskeygenassist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 197, 58], OperandSize::Qword)
}

fn vaeskeygenassist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDI, 1462450878, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 143, 190, 58, 43, 87, 63], OperandSize::Qword)
}

