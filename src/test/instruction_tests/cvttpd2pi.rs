use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvttpd2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 192], OperandSize::Word)
}

fn cvttpd2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 118, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 113, 118], OperandSize::Word)
}

fn cvttpd2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 234], OperandSize::Dword)
}

fn cvttpd2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 691935795, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 4, 245, 51, 26, 62, 41], OperandSize::Dword)
}

fn cvttpd2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 197], OperandSize::Qword)
}

fn cvttpd2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 35], OperandSize::Qword)
}

